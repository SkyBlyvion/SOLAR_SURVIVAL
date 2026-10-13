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
                found = re.search(r'(0x[0-9a-f]+) "Solar Survival · 0.2 · Mars"',tree)
                if found:
                    focused = found.group(1)
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
