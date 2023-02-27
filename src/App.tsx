import { Main } from '@/pages/Main'
import { StreamdeckListProvider } from '@/contexts/StreamdeckList';

function App() {
  return (
    <StreamdeckListProvider>
      <Main />
    </StreamdeckListProvider>
  );
}

export default App;
