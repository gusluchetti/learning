const Pet = ({ id, name, animal, breed, images, location }) => {
  let hero = "https://pets-images.dev-apis.com/pets/none.jpg";
  if (images.length) {
    hero = images[0];
  }

  return (
    <a href={`/details/${id}`} className="pet">
        <div className="img-container">
          <img src={hero} alt={name} />
        </div>
        <div className="pet-info">
          <p>{name}</p>
          <p>{`${animal} - ${breed}`}</p>
          <p>{location}</p>
        </div>
    </a>
  );
};

export default Pet;
