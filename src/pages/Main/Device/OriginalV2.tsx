import { Streamdeck } from "@/model/Streamdeck";
import { Button } from "./Button";

export function OriginalV2(props:{ streamdeck: Streamdeck }) {
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