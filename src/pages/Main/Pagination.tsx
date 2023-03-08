import { Page } from "./Pagination/Page";

export function Pagination() {
  return (
    <div className="justify-center my-8 select-none flex">
      Pages:
      <span className='rounded-full ml-2 bg-zinc-800/40 shadow-md'>
        <Page current={false}>1</Page>
        <Page current={true}>2</Page>
        <button className="py-1 px-3 text-white text-sm mr-2">+</button>
      </span>
    </div>
  );
}