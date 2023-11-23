import Pet from "./Pet";

const Pets = ({ pets }) => {
  return (
    <div className="pets">
      {!pets.length ? (
        <h1>No Pets Found!</h1>
      ) : (
        pets.map((pet) => (
          <Pet
            key={pet.id}
            id={pet.id}
            name={pet.name}
            animal={pet.animal}
            breed={pet.breed}
            images={pet.images}
            location={`${pet.city}, ${pet.state}`}
          />
        ))
      )}
    </div>
  );
};

export default Pets;
