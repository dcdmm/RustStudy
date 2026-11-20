import Greet from "./a_base";
import Counter from "./b_state";
import SlowTask from "./c_async";
import TaskProgress from "./d_channel";
import Ticker from "./e_event";
import PluginDemo from "./f_plugin";
import "./App.css";

function App() {
  return (
    <main className="container">
      <Greet />
      <hr />
      <Counter />
      <hr />
      <SlowTask />
      <hr />
      <TaskProgress />
      <hr />
      <Ticker />
      <hr />
      <PluginDemo />
    </main>
  );
}

export default App;
