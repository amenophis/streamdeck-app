import { ActionsList } from "@/pages/Main/ActionsList";
import { Canvas } from "@/pages/Main/Canvas";
import { PropertyInspector } from "@/pages/Main/PropertyInspector";

import "./Main.css";

export function Main() {
  return (
    <div className='select-none flex flex-row h-screen bg-[#2d2d2d] text-white'>
      <div className='flex flex-col h-screen flex-grow p-10'>
        <Canvas />
        <PropertyInspector />
      </div>
      <ActionsList />
    </div>
  );
}