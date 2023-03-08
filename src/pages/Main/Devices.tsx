import { StreamdeckContext } from "@/contexts/StreamdeckList";
import { Streamdeck } from "@/model/Streamdeck";
import { useContext, useState } from "react";
import { BiChevronDown, BiChevronUp } from 'react-icons/bi';

export function Devices() {
  const [isOpen, setOpen] = useState(false);

  const { streamdeckList, streamdeck: current, select} = useContext(StreamdeckContext);

  return (
    <div className='relative' tabIndex={0} onFocus={() => setOpen(true)} onBlur={() => setOpen(false)}>
      <span className='font-bold inline-block cursor-pointer'>
        {current ? `${current.name} (${current.serial})` : 'Select a Streamdeck'}
        { isOpen 
          && <BiChevronUp className="inline w-5 h-5 ml-2"></BiChevronUp>
          || <BiChevronDown className="inline w-5 h-5 ml-2"></BiChevronDown>
        }
      </span>
      {isOpen &&
        <div className='absolute bg-zinc-600 drop-shadow-md p-2 z-10' style={{ 'minWidth': "300px" }}>
          <div className='text-zinc-400  pl-6'>Appareils</div>
          <ul>
            {
              Array.from(streamdeckList.values()).map((streamdeck) => 
                <li className={'pl-6' + (streamdeck === current) ? 'font-bold' : ''} key={streamdeck.serial}>
                  <a onClick={(e) => select(streamdeck)}>
                    {`${streamdeck.name} (${streamdeck.serial})`}
                  </a>
                </li>
              )
            }
          </ul>
        </div>
      }
    </div>
  );
}

