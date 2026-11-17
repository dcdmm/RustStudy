import Greet from "./demos/Greet";
import Divide from "./demos/Divide";
import Counter from "./demos/Counter";
import Countdown from "./demos/Countdown";
import "./App.css";

// 每个demo组件对应src-tauri/src/lib.rs中的一个示例
function App() {
  return (
    <main className="container">
      <h1>Tauri 2 学习案例</h1>
      <Greet />
      <Divide />
      <Counter />
      <Countdown />
    </main>
  );
}

export default App;
