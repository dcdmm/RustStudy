import Greet from "./a_base";
import Counter from "./b_state";
import "./App.css";

function App() {
  return (
    <main className="container">
      <Greet />
      <hr />
      <Counter />
    </main>
  );
}

export default App;
