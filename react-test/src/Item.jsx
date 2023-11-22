const Item = (props) => {
  return (
    <div>
      <h2>{props.title}</h2>
      <p>{props.info}</p>
      <p><strong>{props.extra}</strong></p>
    </div>
  );
};

export default Item;
