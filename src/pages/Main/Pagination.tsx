export function Pagination() {
  return (
    <div className="justify-center my-8 select-none flex">
      Pages:
      <span className='rounded-full ml-2 bg-zinc-800/40 shadow-md'>
        <button className="py-1 px-5 rounded-full text-zinc-700 bg-white text-sm mr-2">1</button>
        <button className="py-1 px-5 text-white text-sm mr-2">+</button>
      </span>
    </div>
  );
}