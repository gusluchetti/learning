import React from "react";
import ReactDOM from "react-dom/client";
import SearchParams from "./SearchParams";
import { Link, BrowserRouter, Routes, Route } from "react-router-dom";
import Details from "./Details";

const App = () => {
  return (
    <BrowserRouter>
      <header>
        <Link to="/">Adoption Center</Link>
      </header>
      <Routes>
        <Route path="/" element={<SearchParams />} />
        <Route path="/details/:id" element={<Details />} />
      </Routes>
    </BrowserRouter>
  );
};

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
