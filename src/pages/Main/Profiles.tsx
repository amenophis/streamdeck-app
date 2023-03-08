import { useState } from "react";
import { BiChevronDown, BiChevronUp } from 'react-icons/bi';

export function Profiles() {
  const [isOpen, setOpen] = useState(false);

  return (
    <div className='relative mt-1' tabIndex={1} onFocus={() => setOpen(true)} onBlur={() => setOpen(false)}>
      <span className='inline-block cursor-pointer'>
        Profil 1
          { isOpen 
            && <BiChevronUp className="inline w-5 h-5 ml-2"></BiChevronUp>
            || <BiChevronDown className="inline w-5 h-5 ml-2"></BiChevronDown>
          }
      </span>
      {
        isOpen &&
        <div className='absolute bg-zinc-600 drop-shadow-md p-2 z-10' style={{ 'minWidth': "300px" }}>
          <div className='text-zinc-400 pl-6'>Profils</div>
          <ul>
            <li className={'pl-6'}>Profil 1</li>
            <li className={'pl-6'}>Profil 2</li>
          </ul>
          <hr className='mb-2' />
          <ul>
            <li className={'pl-6'}>Nouveau profil</li>
            <li className={'pl-6'}>Modifier les profils...</li>
          </ul>
        </div>
      }
    </div>
  );
}