import { Streamdeck } from "@/model/Streamdeck";
import { Button } from "./Button";

type OriginalV2Props = {
  streamdeck: Streamdeck
};

export function OriginalV2(props: OriginalV2Props) {
  return (
    <div>
      {
        <div className={'pt-5 place-content-center grid grid-cols-[repeat(5,80px)] h-full gap-5'}>
          <Button />
          <Button />
          <Button />
          <Button />
          <Button />
          <Button />
          <Button />
          <Button />
          <Button />
          <Button />
          <Button />
          <Button />
          <Button />
          <Button />
          <Button />
        </div>
      }
    </div>
  );
}