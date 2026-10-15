"""Rééchantillonnage offline des cartes orbitales additionnelles, sans modèle en jeu."""
from pathlib import Path
import json
import struct
import numpy as np
from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
MASTER = ROOT/'art/p7-masters'


def periodic_longitude(image, width=8):
    """Raccorder artistiquement les bords du composite, sans toucher au maître.
    Correction RGB linéaire sur huit colonnes par bord (4° à 720 px).
    Le reste de la carte et ses détails restent exactement inchangés.
    """
    source=np.asarray(image.convert('RGB'),dtype=np.float32)/255
    linear=np.where(source<=.04045,source/12.92,((source+.055)/1.055)**2.4)
    result=linear.copy()
    middle=(linear[:,0]+linear[:,-1])*.5
    for i in range(width):
        t=i/(width-1);weight=1-t*t*(3-2*t)
        result[:,i]+=(middle-linear[:,0])*weight
        result[:,-1-i]+=(middle-linear[:,-1])*weight
    result=np.clip(result,0,1)
    encoded=np.where(result<=.0031308,result*12.92,1.055*result**(1/2.4)-.055)
    output=np.uint8(np.clip(np.rint(encoded*255),0,255))
    assert np.array_equal(output[:,0],output[:,-1])
    assert np.array_equal(output[:,width:-width],np.asarray(image.convert('RGB'))[:,width:-width])
    return Image.fromarray(output)


def mercury_map():
    """Rayons unitaires contre le mesh VTAD : convertir son atlas, pas ses zones blanches.
    Y-up et zéro +X / est -Z sont les conventions du jeu, pas un repère IAU certifié.
    Les poids barycentriques donnent exactement les UV du mesh source au rayon.
    """
    raw=(MASTER/'mercury.glb').read_bytes()
    length,_=struct.unpack_from('<II',raw,12)
    gltf=json.loads(raw[20:20+length]);offset=20+length
    size,_=struct.unpack_from('<II',raw,offset);buffer=raw[offset+8:offset+8+size]
    def accessor(index):
        a=gltf['accessors'][index];b=gltf['bufferViews'][a['bufferView']]
        dtype={5126:'<f4',5123:'<u2',5125:'<u4'}[a['componentType']]
        components={'SCALAR':1,'VEC2':2,'VEC3':3}[a['type']]
        assert 'byteStride' not in b
        start=b.get('byteOffset',0)+a.get('byteOffset',0)
        return np.frombuffer(buffer,dtype=dtype,count=a['count']*components,offset=start).reshape(-1,components)
    primitive=gltf['meshes'][0]['primitives'][0]
    points=accessor(primitive['attributes']['POSITION']).astype(np.float64)
    uv=accessor(primitive['attributes']['TEXCOORD_0']).astype(np.float64)
    indices=accessor(primitive['indices']).reshape(-1,3)
    source=np.asarray(Image.open(MASTER/'mercury-atlas.png').convert('RGB'),dtype=np.float64)
    w,h=1024,512
    longitude=(np.arange(w)+.5)/w*2*np.pi-np.pi
    latitude=np.pi/2-(np.arange(h)+.5)/h*np.pi
    lon,lat=np.meshgrid(longitude,latitude)
    directions=np.stack([np.cos(lat)*np.cos(lon),np.sin(lat),-np.cos(lat)*np.sin(lon)],axis=2)
    output=np.zeros((h,w,3),dtype=np.uint8);covered=np.zeros((h,w),dtype=bool)
    for tri in indices:
        vertices=points[tri];n=vertices/np.linalg.norm(vertices,axis=1)[:,None]
        uu=.5+np.arctan2(-n[:,2],n[:,0])/(2*np.pi)
        vv=np.arccos(np.clip(n[:,1],-1,1))/np.pi
        if np.ptp(uu)>.5:uu[uu<.5]+=1
        for i in range(3):
            if abs(n[i,1])>.999999:uu[i]=(uu[(i+1)%3]+uu[(i+2)%3])/2
        x0=int(np.floor(uu.min()*w))-1;x1=int(np.ceil(uu.max()*w))+1
        y0=max(0,int(np.floor(vv.min()*h))-1);y1=min(h,int(np.ceil(vv.max()*h))+1)
        # Près des pôles, la longitude peut balayer tout le tour à l'intérieur
        # d'un triangle. Bounding box conservative ; le test rayon reste exact.
        if vv.min()<.125:
            x0,x1,y0=0,w,0
        if vv.max()>.875:
            x0,x1,y1=0,w,h
        if x1<=x0 or y1<=y0:continue
        xs=np.arange(x0,x1)%w;ys=np.arange(y0,y1)
        rays=directions[np.ix_(ys,xs)]
        a,b,c=vertices;e1=b-a;e2=c-a
        cross=np.cross(rays,e2);det=cross@e1
        safe=np.where(abs(det)>1e-12,det,1)
        u=(cross@(-a))/safe;q=np.cross(-a,e1);v=(rays@q)/safe;t=np.dot(e2,q)/safe
        valid=(abs(det)>1e-12)&(u>=-1e-7)&(v>=-1e-7)&(u+v<=1+1e-7)&(t>0)
        texture_uv=(1-u-v)[...,None]*uv[tri[0]]+u[...,None]*uv[tri[1]]+v[...,None]*uv[tri[2]]
        # glTF : origine d'image en haut à gauche. Les UV de l'atlas sont conservées.
        tx=np.clip(texture_uv[...,0]*source.shape[1]-.5,0,source.shape[1]-1)
        ty=np.clip(texture_uv[...,1]*source.shape[0]-.5,0,source.shape[0]-1)
        ix=np.floor(tx).astype(int);iy=np.floor(ty).astype(int)
        jx=np.minimum(ix+1,source.shape[1]-1);jy=np.minimum(iy+1,source.shape[0]-1)
        fx=(tx-ix)[...,None];fy=(ty-iy)[...,None]
        color=(source[iy,ix]*(1-fx)+source[iy,jx]*fx)*(1-fy)+(source[jy,ix]*(1-fx)+source[jy,jx]*fx)*fy
        yy,xx=np.meshgrid(ys,xs,indexing='ij')
        output[yy[valid],xx[valid]]=np.uint8(np.clip(np.rint(color[valid]),0,255));covered[yy[valid],xx[valid]]=True
    if not covered.all():
        yy,xx=np.where(~covered); print('Mercure manquants : lignes',int(yy.min()),int(yy.max()),'bandes',[int(np.count_nonzero(~covered[i:i+64])) for i in range(0,h,64)],flush=True)
    assert covered.all(), f'Mercure : {np.count_nonzero(~covered)} pixels sans couverture'
    return Image.fromarray(output)


def prepare(export):
    records=[export('mercury',mercury_map(),False)]
    for name,source in [('venus','venus-clouds.png'),('jupiter','jupiter.jpg'),('saturn','saturn.jpg'),('uranus','uranus.png'),('neptune','neptune.jpg')]:
        im=Image.open(MASTER/source).convert('RGB')
        assert im.width==2*im.height
        size=(min(2048,im.width),min(1024,im.height))
        im=im.resize(size,Image.Resampling.LANCZOS)
        if name=='jupiter': im=periodic_longitude(im)
        records.append(export(name,im,False))
    return records
