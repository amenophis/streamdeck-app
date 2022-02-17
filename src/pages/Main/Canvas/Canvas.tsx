import { useState } from "react";
import "./Canvas.css";


export function Canvas() {
  const [deviceListOpened, setDeviceListOpened] = useState(false);
  const [profileListOpened, setProfileListOpened] = useState(false);

  return (
    <div className='mx-30'>
      {/* Dorpdown: https://stackoverflow.com/questions/32553158/detect-click-outside-react-component */}
      <div className=''>
          <div className='relative' tabIndex={0} onFocus={() => setDeviceListOpened(true)} onBlur={() => setDeviceListOpened(false)}>
              <span className='font-bold inline-block cursor-pointer'>
                  Stream Deck 
                  {/* <ChevronDownIcon className="inline w-5 h-5 ml-2"></ChevronDownIcon> */}
              </span>
              { 
                  deviceListOpened && 
                  <div className='absolute bg-zinc-600 drop-shadow-md p-2 z-10' style={{'minWidth': "300px"}}>
                      <div className='text-zinc-400  pl-6'>Appareils</div>
                      <ul>
                          {/* <li><CheckIcon className="inline w-5 h-5 mr-1"></CheckIcon>Stream Deck 1</li> */}
                          <li className={ 'pl-6' }>Stream Deck 2</li>
                      </ul>
                  </div>
              }
              
          </div>
          <div className='relative mt-1' tabIndex={1} onFocus={() => setProfileListOpened(true)} onBlur={() => setProfileListOpened(false)}>
              <span className='inline-block cursor-pointer'>
                  Profil 1
                  {/* <ChevronDownIcon className="inline w-5 h-5 ml-2"></ChevronDownIcon> */}
              </span>
              { 
                  profileListOpened && 
                  <div className='absolute bg-zinc-600 drop-shadow-md p-2 z-10' style={{'minWidth': "300px"}}>
                      <div className='text-zinc-400 pl-6'>Profils</div>
                      <ul>
                          {/* <li><CheckIcon className="inline w-5 h-5 mr-1"></CheckIcon>Profil 1</li> */}
                          <li className={ 'pl-6' }>Profil 2</li>
                      </ul>
                      <hr className='mb-2'/>
                      <ul>
                          <li className={ 'pl-6' }>Nouveau profil</li>
                          <li className={ 'pl-6' }>Modifier les profils...</li>
                      </ul>
                  </div>
              }
              
          </div>
      </div>
      <div>
      <div className='pt-5 place-content-center grid grid-cols-[repeat(5,80px)] h-full gap-4'>
      {
          [...Array(15).keys()].map((i) => 
          <div className='w-20 h-20 border-3 border-zinc-600 rounded-2xl bg-zinc-800'>
              test {i}
          </div>
          )
      }
      </div>
      </div>
      <div className="justify-center my-8 select-none flex">
          Pages: 
          <span className='rounded-full ml-2 bg-zinc-800/40 shadow-md'>
              <button className="py-1 px-5 rounded-full text-zinc-700 bg-white text-sm mr-2">1</button>
              <button className="py-1 px-5 text-white text-sm mr-2">+</button>
          </span>
      </div>
  </div>
  );
}