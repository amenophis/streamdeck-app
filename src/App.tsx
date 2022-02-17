import "./App.css";

import { Main } from '@/pages/Main'

function App() {
  return (
    <div onContextMenu={(e) => {
      e.preventDefault(); // prevent the default behaviour when right clicked
    }}>
      <Main />
    </div>
  );
}

export default App;
