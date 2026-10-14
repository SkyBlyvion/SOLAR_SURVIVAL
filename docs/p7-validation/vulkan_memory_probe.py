"""VK_EXT_memory_budget, lecture facultative sans créer de device/pipeline.
heapUsage concerne le processus observateur, PAS le jeu. Il ne mesure donc pas la VRAM du jeu.
"""
import ctypes as c

class AppInfo(c.Structure):
    _fields_=[('sType',c.c_uint32),('pNext',c.c_void_p),('pApplicationName',c.c_char_p),('applicationVersion',c.c_uint32),('pEngineName',c.c_char_p),('engineVersion',c.c_uint32),('apiVersion',c.c_uint32)]
class InstanceInfo(c.Structure):
    _fields_=[('sType',c.c_uint32),('pNext',c.c_void_p),('flags',c.c_uint32),('pApplicationInfo',c.POINTER(AppInfo)),('layerCount',c.c_uint32),('layers',c.c_void_p),('extensionCount',c.c_uint32),('extensions',c.c_void_p)]
class MemoryType(c.Structure):
    _fields_=[('flags',c.c_uint32),('heap',c.c_uint32)]
class Heap(c.Structure):
    _fields_=[('size',c.c_uint64),('flags',c.c_uint32)]
class MemoryProperties(c.Structure):
    _fields_=[('typeCount',c.c_uint32),('types',MemoryType*32),('heapCount',c.c_uint32),('heaps',Heap*16)]
class Budget(c.Structure):
    _fields_=[('sType',c.c_uint32),('pNext',c.c_void_p),('budget',c.c_uint64*16),('usage',c.c_uint64*16)]
class Properties2(c.Structure):
    _fields_=[('sType',c.c_uint32),('pNext',c.c_void_p),('properties',MemoryProperties)]
class Extension(c.Structure):
    _fields_=[('name',c.c_char*256),('version',c.c_uint32)]

class VulkanMemory:
    def __init__(self):
        self.instance=c.c_void_p()
        self.lib=c.CDLL('libvulkan.so.1')
        app=AppInfo(sType=0,pApplicationName=b'Solar memory diagnostic',apiVersion=(1<<22)|(1<<12))
        info=InstanceInfo(sType=1,pApplicationInfo=c.pointer(app))
        self.lib.vkCreateInstance.argtypes=[c.POINTER(InstanceInfo),c.c_void_p,c.POINTER(c.c_void_p)]
        if self.lib.vkCreateInstance(c.byref(info),None,c.byref(self.instance))!=0:raise RuntimeError('vkCreateInstance indisponible')
        self.lib.vkEnumeratePhysicalDevices.argtypes=[c.c_void_p,c.POINTER(c.c_uint32),c.POINTER(c.c_void_p)]
        count=c.c_uint32();self.lib.vkEnumeratePhysicalDevices(self.instance,c.byref(count),None)
        devices=(c.c_void_p*count.value)();self.lib.vkEnumeratePhysicalDevices(self.instance,c.byref(count),devices)
        self.lib.vkGetPhysicalDeviceProperties.argtypes=[c.c_void_p,c.c_void_p]
        self.lib.vkEnumerateDeviceExtensionProperties.argtypes=[c.c_void_p,c.c_char_p,c.POINTER(c.c_uint32),c.POINTER(Extension)]
        self.lib.vkGetPhysicalDeviceMemoryProperties2.argtypes=[c.c_void_p,c.POINTER(Properties2)]
        self.lib.vkDestroyInstance.argtypes=[c.c_void_p,c.c_void_p]
        self.device=None
        for device in devices:
            props=c.create_string_buffer(4096);self.lib.vkGetPhysicalDeviceProperties(device,props)
            name=props.raw[20:276].split(b'\0')[0].decode()
            ext_count=c.c_uint32();self.lib.vkEnumerateDeviceExtensionProperties(device,None,c.byref(ext_count),None)
            extensions=(Extension*ext_count.value)();self.lib.vkEnumerateDeviceExtensionProperties(device,None,c.byref(ext_count),extensions)
            if 'GTX 1060' in name and any(x.name==b'VK_EXT_memory_budget' for x in extensions):
                self.device=device;self.name=name;break
        if self.device is None:self.close();raise RuntimeError('VK_EXT_memory_budget absent sur GTX 1060')

    def sample(self):
        budget=Budget(sType=1000237000)
        properties=Properties2(sType=1000059006,pNext=c.addressof(budget))
        self.lib.vkGetPhysicalDeviceMemoryProperties2(self.device,c.byref(properties))
        return dict(device=self.name,heaps=[dict(index=i,size_bytes=properties.properties.heaps[i].size,
            device_local=bool(properties.properties.heaps[i].flags&1),budget_bytes=budget.budget[i],observer_usage_bytes=budget.usage[i])
            for i in range(properties.properties.heapCount)])

    def close(self):
        if self.instance.value:self.lib.vkDestroyInstance(self.instance,None);self.instance=c.c_void_p()

if __name__=='__main__':
    import json
    try:
        probe=VulkanMemory();print(json.dumps(probe.sample(),indent=2));probe.close()
    except (OSError,RuntimeError) as e:print(json.dumps(dict(unavailable=str(e))))
