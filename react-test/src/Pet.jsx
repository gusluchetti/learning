const Pet = (props) => {
  return (
    <div className="pet">
      <p>
        {props.name} ({props.animal})
      </p>
      <p>breed: {props.breed}</p>
    </div>
  );
};

export default Pet;
