import { createContext, PropsWithChildren, useEffect, useState } from 'react';
import { Streamdeck } from '@/model/Streamdeck';

import { Event, listen } from '@tauri-apps/api/event'

interface IStreamdeckContext {
  streamdeckList: Map<string, Streamdeck>,
  streamdeck: Streamdeck|null;
  select: (streamdeck: Streamdeck|null) => void;
}

export const StreamdeckContext = createContext<IStreamdeckContext>({
  streamdeckList: new Map(),
  streamdeck: null,
  select: (streamdeck: Streamdeck|null) => {} // This function is overrided by the setCurrent below
});

type StreamdeckKeyPayload = {
  serial: string,
  key: number
};

export const StreamdeckListProvider = (props: PropsWithChildren) => {
  const [streamdeckList, setStreamdeckList] = useState<Map<string, Streamdeck>>(new Map());
  const [streamdeck, select] = useState<Streamdeck|null>(null);
  
  useEffect(() => {
    const attached_unlisten = listen('device_attached', (event: Event<Streamdeck>) => {
      const addedStreamdeck = new Streamdeck(
        event.payload.kind,
        event.payload.name,
        event.payload.serial,
        event.payload.row_count,
        event.payload.column_count,
        event.payload.key_count,
      );

      streamdeckList.set(event.payload.serial, addedStreamdeck);

      setStreamdeckList(streamdeckList);

      if (!streamdeck) {
        select(addedStreamdeck);
      }
    });

    const detached_unlisten = listen('device_detached', (event: Event<string>) => {
      const removedStreamdeck = streamdeckList.get(event.payload);
      streamdeckList.delete(event.payload);

      if (streamdeck === removedStreamdeck) {
        const firstStreamdeck = streamdeckList.values().next().value;
        select(firstStreamdeck);
      }

      setStreamdeckList(streamdeckList);
    });

    const key_up_unlisten = listen('key_up', (event: Event<StreamdeckKeyPayload>) => {
      console.log(event.payload);
    });

    const key_down_unlisten = listen('key_down', (event: Event<StreamdeckKeyPayload>) => {
      console.log(event.payload);
    });

    return () => {
      attached_unlisten.then((unlisten) => {
        unlisten();
      });
      
      detached_unlisten.then((unlisten) => {
        unlisten();
      });
      
      key_up_unlisten.then((unlisten) => {
        unlisten();
      });
      
      key_down_unlisten.then((unlisten) => {
        unlisten();
      });
    }
  }, [streamdeckList, streamdeck]);

  return (
    <StreamdeckContext.Provider
      value={{streamdeckList, streamdeck: streamdeck, select: select}}
    >
      {props.children}
    </StreamdeckContext.Provider>
  );
};