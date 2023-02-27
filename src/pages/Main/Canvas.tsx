import { Pagination } from "./Pagination";
import { Devices } from "./Devices";
import { Profiles } from "./Profiles";
import { useContext } from "react";
import { StreamdeckContext } from "@/contexts/StreamdeckList";
import { OriginalV2 } from "./Device/OriginalV2";

export function Canvas() {
  const { current } = useContext(StreamdeckContext);

  return (
    <div className='mx-30'>
      <div className=''>
        <Devices />
        <Profiles />
      </div>
      {
        current?.kind === "original_v2" && <OriginalV2 streamdeck={current} />
      }
      <Pagination />
    </div>
  );
}