const Item = (props) => {
  return React.createElement("div", null, [
    React.createElement("h2", null, props.title),
    React.createElement("p", null, props.info),
    React.createElement("p", null, props.extra),
  ]);
};

const App = () => {
  return React.createElement("div", null, [
    React.createElement("h1", null, "Hello Basic React!"),
    React.createElement(Item, { title: "test", info: "nothing", extra: "" }),
    React.createElement(Item, {
      title: "spooky",
      info: "halloween",
      extra: "?",
    }),
  ]);
};

const container = document.getElementById("root");
const root = ReactDOM.createRoot(container);
root.render(React.createElement(App));
