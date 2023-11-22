const Item = (props) => {
	return (
		<div>
			<h2>{props.title}</h2>
			<p>{props.info}</p>
			<p>{props.extra}</p>
		</div>
	);
};

export default Item;
