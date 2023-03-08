import { Pagination } from "./Pagination";
import { Devices } from "./Devices";
import { Profiles } from "./Profiles";
import { useContext } from "react";
import { StreamdeckContext } from "@/contexts/StreamdeckList";
import { OriginalV2 } from "./Device/OriginalV2";
import { NoDevice } from "./Device/NoDevice";
import { Kind, Streamdeck } from "@/model/Streamdeck";

export function Canvas() {
  const { streamdeck: current } = useContext(StreamdeckContext);

  function device(streamdeck: Streamdeck|null)
  {
    if (!streamdeck) {
      return <NoDevice />
    } else if (Kind.OriginalV2 === streamdeck.kind) {
      return <OriginalV2 streamdeck={streamdeck} />
    }
  }

  return (
    <div className='mx-30'>
      <>
        <div className=''>
          <Devices />
          <Profiles />
        </div>
        {device(current)}
        <Pagination />
      </>
    </div>
  );
}
