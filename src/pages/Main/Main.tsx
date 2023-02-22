import { ActionsList } from "@/pages/Main/ActionsList";
import { Canvas } from "@/pages/Main/Canvas";
import { PropertyInspector } from "@/pages/Main/PropertyInspector";
import { w } from 'windstitch';

const Wrapper = w.div(`select-none flex flex-row h-screen bg-[#2d2d2d] text-white`);
const LeftPane = w.div(`flex flex-col h-screen flex-grow p-10`);

export function Main() {
  return (
    <Wrapper>
      <LeftPane>
        <Canvas />
        <PropertyInspector />
      </LeftPane>
      <ActionsList />
    </Wrapper>
  );
}