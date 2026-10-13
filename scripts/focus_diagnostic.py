"""Focus réel X11 d'une fenêtre de diagnostic, sans simuler clavier/souris."""
import ctypes
import os
import re
import subprocess
import time


def run_focused(command, env, log, timeout):
    process = subprocess.Popen(command, env=env, stdout=log, stderr=subprocess.STDOUT)
    start = time.monotonic()
    focused = None
    if os.environ.get('DISPLAY'):
        xlib = ctypes.CDLL('libX11.so.6')
        xlib.XOpenDisplay.restype = ctypes.c_void_p
        xlib.XRaiseWindow.argtypes = [ctypes.c_void_p, ctypes.c_ulong]
        xlib.XSetInputFocus.argtypes = [ctypes.c_void_p, ctypes.c_ulong, ctypes.c_int, ctypes.c_ulong]
        xlib.XFlush.argtypes = [ctypes.c_void_p]
        xlib.XCloseDisplay.argtypes = [ctypes.c_void_p]
        display = xlib.XOpenDisplay(None)
        try:
            while process.poll() is None and time.monotonic()-start < 5:
                tree = subprocess.check_output(['xwininfo','-root','-tree'],text=True)
                found = re.search(r'(0x[0-9a-f]+) "Solar Survival · 0.2 · Mars": \("solar_survival"',tree)
                if found and time.monotonic()-start >= 2.0:
                    focused = found.group(1)
                    class ClientData(ctypes.Union):
                        _fields_ = [('b', ctypes.c_char*20), ('s', ctypes.c_short*10), ('l', ctypes.c_long*5)]
                    class ClientEvent(ctypes.Structure):
                        _fields_ = [('type',ctypes.c_int),('serial',ctypes.c_ulong),('send_event',ctypes.c_int),('display',ctypes.c_void_p),('window',ctypes.c_ulong),('message_type',ctypes.c_ulong),('format',ctypes.c_int),('data',ClientData)]
                    class Event(ctypes.Union):
                        _fields_ = [('client',ClientEvent),('pad',ctypes.c_long*24)]
                    xlib.XInternAtom.argtypes=[ctypes.c_void_p,ctypes.c_char_p,ctypes.c_int];xlib.XInternAtom.restype=ctypes.c_ulong
                    xlib.XDefaultRootWindow.argtypes=[ctypes.c_void_p];xlib.XDefaultRootWindow.restype=ctypes.c_ulong
                    xlib.XSendEvent.argtypes=[ctypes.c_void_p,ctypes.c_ulong,ctypes.c_int,ctypes.c_long,ctypes.POINTER(Event)]
                    event=Event();event.client.type=33;event.client.display=display;event.client.window=int(focused,16)
                    event.client.message_type=xlib.XInternAtom(display,b'_NET_ACTIVE_WINDOW',0);event.client.format=32
                    event.client.data.l[0]=2
                    xlib.XSendEvent(display,xlib.XDefaultRootWindow(display),0,(1<<19)|(1<<20),ctypes.byref(event))
                    xlib.XRaiseWindow(display,int(focused,16))
                    xlib.XSetInputFocus(display,int(focused,16),1,0)
                    xlib.XFlush(display)
                    break
                time.sleep(.1)
        finally:
            xlib.XCloseDisplay(display)
    try:
        code = process.wait(timeout=max(1,timeout-(time.monotonic()-start)))
    except subprocess.TimeoutExpired:
        process.kill();process.wait();raise
    return code, focused
