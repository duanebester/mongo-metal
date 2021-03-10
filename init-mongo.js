var state1 = ObjectId();
db.state.insert({ _id:state1, name: 'TX', });
var state2 = ObjectId();
db.state.insert({ _id:state2, name: 'CA', });
var state3 = ObjectId();
db.state.insert({ _id:state3, name: 'SC', });
var state4 = ObjectId();
db.state.insert({ _id:state4, name: 'TA', });
var state5 = ObjectId();
db.state.insert({ _id:state5, name: 'NL', });

var country1 = ObjectId();
db.country.insert({ _id: country1, code: 1, name: 'USA', states: { $ref: 'state', $ids: [state1, state2] } });

var country2 = ObjectId();
db.country.insert({ _id: country2, code: 3, name: 'Brasil', states: { $ref: 'state', $ids: [state3] } });

var country3 = ObjectId();
db.country.insert({ _id: country3, code: 2, name: 'Mexico', states: { $ref: 'state', $ids: [state4, state5] } });

var planet1 = ObjectId();
db.planet.insert({ _id: planet1, name: 'Earth', countries: {$ref: 'country', $ids: [country1, country2, country3] } });
