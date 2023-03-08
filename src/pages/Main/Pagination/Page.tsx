import { PropsWithChildren } from "react";

type PageProps = {
  current: boolean
}

export function Page({ current, children }: PropsWithChildren<PageProps>) {
  let classes = 'py-1 px-4 text-zinc-700 text-sm mr-2';
  
  if (current) {
    classes += ' rounded-full bg-white';
  }
  
  return (  
    <button className={classes}>{children}</button>
  );
}