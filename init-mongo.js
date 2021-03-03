db.createCollection("countries");
db.createCollection("states");
db.createCollection("planets");

var state1 = ObjectId();
db.states.insert({ _id:state1, name: 'TX', });
var state2 = ObjectId();
db.states.insert({ _id:state2, name: 'CA', });
var state3 = ObjectId();
db.states.insert({ _id:state3, name: 'SC', });
var state4 = ObjectId();
db.states.insert({ _id:state4, name: 'TA', });
var state5 = ObjectId();
db.states.insert({ _id:state5, name: 'NL', });

var country1 = ObjectId();
db.countries.insert({ _id: country1, name: 'USA', states: { $ref: 'states', $ids: [state1, state2] } });

var country2 = ObjectId();
db.countries.insert({ _id: country2, name: 'Brasil', states: { $ref: 'states', $ids: [state3] } });

var country3 = ObjectId();
db.countries.insert({ _id: country3, name: 'Mexico', states: { $ref: 'states', $ids: [state4, state5] } });

var planet1 = ObjectId();
db.planets.insert({ _id: planet1, name: 'Earth', countries: {$ref: 'countries', $ids: [country1, country2, country3] } });
