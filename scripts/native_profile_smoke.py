#!/usr/bin/env python3
"""Entrées clavier/souris X11 réelles sur une archive et un profil d'essai déjà payé, sans trajet."""
import argparse,ctypes,hashlib,json,os,re,signal,subprocess,time
from pathlib import Path

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--binary',type=Path,required=True);p.add_argument('--cwd',type=Path,required=True)
    p.add_argument('--save-root',type=Path,required=True);p.add_argument('--output',type=Path,required=True)
    p.add_argument('--allow-desktop-input',action='store_true',required=True)
    a=p.parse_args();out=a.output.resolve();out.mkdir()
    root=a.save_root.resolve();profile=root/'Grouped Pending/session.solar';before=profile.read_text()
    assert re.search(r'^expedition 1 false ',before,re.M), 'Profil préparé après paiement requis'
    env={k:v for k,v in os.environ.items() if not k.startswith('SOLAR_')}
    env.update(SOLAR_SAVE_ROOT=str(root),SOLAR_CAPTURE_DIR=str(out),MESA_VK_WSI_PRESENT_MODE='immediate',RUST_LOG='info')
    x=ctypes.CDLL('libX11.so.6');xt=ctypes.CDLL('libXtst.so.6')
    x.XOpenDisplay.restype=ctypes.c_void_p;d=x.XOpenDisplay(None);assert d
    x.XStringToKeysym.argtypes=[ctypes.c_char_p];x.XStringToKeysym.restype=ctypes.c_ulong
    x.XKeysymToKeycode.argtypes=[ctypes.c_void_p,ctypes.c_ulong];x.XKeysymToKeycode.restype=ctypes.c_uint
    x.XFlush.argtypes=[ctypes.c_void_p];x.XCloseDisplay.argtypes=[ctypes.c_void_p]
    x.XRaiseWindow.argtypes=[ctypes.c_void_p,ctypes.c_ulong];x.XSetInputFocus.argtypes=[ctypes.c_void_p,ctypes.c_ulong,ctypes.c_int,ctypes.c_ulong]
    xt.XTestFakeKeyEvent.argtypes=[ctypes.c_void_p,ctypes.c_uint,ctypes.c_int,ctypes.c_ulong]
    xt.XTestFakeButtonEvent.argtypes=[ctypes.c_void_p,ctypes.c_uint,ctypes.c_int,ctypes.c_ulong]
    xt.XTestFakeMotionEvent.argtypes=[ctypes.c_void_p,ctypes.c_int,ctypes.c_int,ctypes.c_int,ctypes.c_ulong]
    def key(name):
        code=x.XKeysymToKeycode(d,x.XStringToKeysym(name.encode()));assert code
        xt.XTestFakeKeyEvent(d,code,1,0);x.XFlush(d);time.sleep(.12)
        xt.XTestFakeKeyEvent(d,code,0,0);x.XFlush(d);time.sleep(.35)
    def shot():key('F12');time.sleep(.6)
    log=(out/'game.log').open('w');proc=subprocess.Popen([str(a.binary.resolve())],cwd=a.cwd.resolve(),env=env,stdout=log,stderr=subprocess.STDOUT)
    result={'binary':str(a.binary.resolve()),'binary_sha256':hashlib.sha256(a.binary.read_bytes()).hexdigest(),'cwd':str(a.cwd.resolve()),'scope':'Native XTest keyboard and right mouse button on the test game only. No walking or expedition pass.'}
    try:
        window=None
        for _ in range(80):
            tree=subprocess.check_output(['xwininfo','-root','-tree'],text=True)
            match=re.search(r'(0x[0-9a-f]+) "Solar Survival · 0.2 · Mars": \("solar_survival"',tree)
            if match:window=int(match.group(1),16);break
            assert proc.poll() is None;time.sleep(.1)
        assert window;time.sleep(2)
        x.XRaiseWindow(d,window);x.XSetInputFocus(d,window,1,0);x.XFlush(d);time.sleep(.4)
        shot();key('F9');shot();key('Return');time.sleep(.6);key('Return');shot()
        key('v');key('Escape');key('F5');shot()
        # Capturer/libérer par les vrais événements de périphérique, puis revenir au menu.
        key('Escape')
        geometry=subprocess.check_output(['xwininfo','-id',hex(window)],text=True)
        left=int(re.search(r'Absolute upper-left X:\s+(-?\d+)',geometry).group(1));top=int(re.search(r'Absolute upper-left Y:\s+(-?\d+)',geometry).group(1))
        width=int(re.search(r'Width:\s+(\d+)',geometry).group(1));height=int(re.search(r'Height:\s+(\d+)',geometry).group(1))
        xt.XTestFakeMotionEvent(d,-1,left+width//2,top+height//2,0);x.XFlush(d);time.sleep(.2)
        xt.XTestFakeButtonEvent(d,3,1,0);x.XFlush(d);time.sleep(.1)
        xt.XTestFakeButtonEvent(d,3,0,0);x.XFlush(d);time.sleep(.3)
        key('Escape');key('Escape');shot()
        # Fermeture WM de notre fenêtre, sans raccourci global ni processus utilisateur.
        class Data(ctypes.Union):_fields_=[('l',ctypes.c_long*5)]
        class Client(ctypes.Structure):_fields_=[('type',ctypes.c_int),('serial',ctypes.c_ulong),('send_event',ctypes.c_int),('display',ctypes.c_void_p),('window',ctypes.c_ulong),('message_type',ctypes.c_ulong),('format',ctypes.c_int),('data',Data)]
        class Event(ctypes.Union):_fields_=[('client',Client),('pad',ctypes.c_long*24)]
        x.XInternAtom.argtypes=[ctypes.c_void_p,ctypes.c_char_p,ctypes.c_int];x.XInternAtom.restype=ctypes.c_ulong
        x.XSendEvent.argtypes=[ctypes.c_void_p,ctypes.c_ulong,ctypes.c_int,ctypes.c_long,ctypes.POINTER(Event)]
        event=Event();event.client.type=33;event.client.display=d;event.client.window=window;event.client.format=32
        event.client.message_type=x.XInternAtom(d,b'WM_PROTOCOLS',0);event.client.data.l[0]=x.XInternAtom(d,b'WM_DELETE_WINDOW',0)
        x.XSendEvent(d,window,0,0,ctypes.byref(event));x.XFlush(d)
        result['exit_code']=proc.wait(timeout=10)
        after=profile.read_text()
        for tag in ['expedition ','bindings ','inventory ','slots ','slot ']:
            old=[l for l in before.splitlines() if l.startswith(tag)];new=[l for l in after.splitlines() if l.startswith(tag)]
            assert old and old==new,(tag,old,new)
        result['same_progress_bindings_inventory']=True
        result['captures']=len(list(out.glob('capture-*.png')))
        assert result['captures']==5
        assert result['exit_code']==0
    finally:
        if proc.poll() is None:proc.terminate();proc.wait(timeout=10)
        x.XCloseDisplay(d);log.close();(out/'result.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result,indent=2))
if __name__=='__main__':main()
