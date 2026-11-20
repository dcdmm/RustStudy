import Greet from "./a_base";
import Counter from "./b_state";
import SlowTask from "./c_async";
import "./App.css";

function App() {
  return (
    <main className="container">
      <Greet />
      <hr />
      <Counter />
      <hr />
      <SlowTask />
    </main>
  );
}

export default App;
