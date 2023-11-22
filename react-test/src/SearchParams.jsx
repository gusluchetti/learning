import { useState } from 'react';

const SearchParams = () => {
  // useState == react hook to add state variable
  // state variable = retain data between render, 
  // update var and trigger re-render
  const [type, setType] = useState("Normal")

  return (
    <div className="search-params">
      <form>
        <label htmlFor="type">
          Type:
          <input 
            id="type" 
            placeholder="Type"
            onChange={(e) => setType(e.target.value)} 
            value={type.toLowerCase()} /> 
        </label>
        <button type="button">Submit</button>
      </form>
    </div>
  );
};

export default SearchParams;
