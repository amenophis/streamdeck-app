import { createContext, PropsWithChildren, useEffect, useState } from 'react';
import { Streamdeck } from '@/model/Streamdeck';

import { Event, listen } from '@tauri-apps/api/event'

interface IStreamdeckContext {
  streamdeckList: Streamdeck[],
  current: Streamdeck|null;
  setCurrent: (streamdeck: Streamdeck|null) => void;
}

export const StreamdeckContext = createContext<IStreamdeckContext>({
  streamdeckList: [],
  current: null,
  setCurrent: (streamdeck: Streamdeck|null) => {} // This function is overrided by the setCurrent below
});

export const StreamdeckListProvider = (props: PropsWithChildren) => {
  const [streamdeckList, setStreamdeckList] = useState<Streamdeck[]>([]);
  const [current, setCurrent] = useState<Streamdeck|null>(null);
  
  useEffect(() => {
    const attached_unlisten = listen('device_attached', (event: Event<Streamdeck>) => {
      streamdeckList.push(new Streamdeck(
        event.payload.kind,
        event.payload.name,
        event.payload.serial,
        event.payload.row_count,
        event.payload.column_count,
        event.payload.key_count,
      ));

      setStreamdeckList(streamdeckList);
    });

    const detached_unlisten = listen('device_detached', (event: Event<string>) => {
      setStreamdeckList(streamdeckList.filter((s) => {
        s.serial !== event.payload;
      }));
    });

    return () => {
      attached_unlisten.then((unlisten) => {
        unlisten();
      });
      
      detached_unlisten.then((unlisten) => {
        unlisten();
      });
    }
  }, [streamdeckList]);

  return (
    <StreamdeckContext.Provider
      value={{streamdeckList, current, setCurrent}}
    >
      {props.children}
    </StreamdeckContext.Provider>
  );
};