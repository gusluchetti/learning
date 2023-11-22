const SearchParams = () => {
  const location = "Kanto";

  return (
    <div className="search-params">
      <form>
        <label htmlFor="location">
          <input id="location" value={location} placeholder="Location" />
        </label>
        <button type="submit">Submit</button>
      </form>
    </div>
  );
};
