import { createRoot } from "react-dom";
import Item from "./Item";

const App = () => {
  return (
    <div>
      <h1>Hello Basic React!</h1>
      <Item title="title" info="info" extra="extra" />
      <Item title="2" info="testing" extra="things" />
    </div>
  );
};

const container = document.getElementById("root");
const root = createRoot(container);
root.render(<App />);
