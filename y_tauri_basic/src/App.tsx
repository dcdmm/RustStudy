import { BrowserRouter, Navigate, NavLink, Route, Routes } from "react-router";
import Greet from "./a_base";
import Counter from "./b_state";
import SlowTask from "./c_async";
import TaskProgress from "./d_channel";
import Ticker from "./e_event";
import PluginDemo from "./f_plugin0_dialog-opener";
import Note from "./f_plugin1_store";
import ReadText from "./g_error";
import PrimeCounter from "./h_heavy";
import TodoPage from "./i_api/TodoPage";
import "./App.css";

const pages = [
  { name: "a_base", element: <Greet /> },
  { name: "b_state", element: <Counter /> },
  { name: "c_async", element: <SlowTask /> },
  { name: "d_channel", element: <TaskProgress /> },
  { name: "e_event", element: <Ticker /> },
  { name: "f_plugin0_dialog-opener", element: <PluginDemo /> },
  { name: "f_plugin1_store", element: <Note /> },
  { name: "g_error", element: <ReadText /> },
  { name: "h_heavy", element: <PrimeCounter /> },
  { name: "i_api", element: <TodoPage /> },
];

function App() {
  return (
    <BrowserRouter>
      <div className="layout">
        <nav className="sidebar">
          {pages.map((page) => (
            <NavLink key={page.name} to={`/${page.name}`}>
              {page.name}
            </NavLink>
          ))}
        </nav>
        <main className="container">
          <Routes>
            {pages.map((page) => (
              <Route key={page.name} path={`/${page.name}`} element={page.element} />
            ))}
            <Route path="*" element={<Navigate to={`/${pages[0].name}`} replace />} />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
}

export default App;
