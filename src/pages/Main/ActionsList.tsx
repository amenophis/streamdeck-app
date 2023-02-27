import { BiSearch } from 'react-icons/bi';

export function ActionsList() {
  return (
    <div className='flex-none w-72 bg-[#292929] pl-px'>
      <div className='bg-zinc-800 mb-px py-3'>
        <label className="relative text-gray-400 flex place-content-center">
          <BiSearch className="pointer-events-none w-5 h-5 absolute top-1/2 transform -translate-y-1/2 left-10" />
          <input type="text" placeholder="Rechercher" className="px-4 pl-8 opacity-10 h-8 rounded-lg border-0" />
        </label>
      </div>
      <div className='bg-zinc-800 h-12'></div>
    </div>
  );
}