import { createRoot } from "react-dom/client";
import Item from "./Item";
import SearchParams from "./SearchParams"

const App = () => {
  return (
    <div>
      <h1>Hello Basic React!</h1>
      <SearchParams />
      <Item title="title" info="info" extra="" />
      <Item title="subtitle" info="testing react" extra="extra info" />
    </div>
  );
};

const container = document.getElementById("root");
const root = createRoot(container);
root.render(<App />);
