db.createCollection("country");
db.createCollection("state");

var country1 = ObjectId();
db.country.insert({ _id: country1, code: 1, name: 'USA' });
db.state.insert({ code: 1, name: 'TX', country : { $ref: 'country', $id: country1 } });
db.state.insert({ code: 2, name: 'CA', country : { $ref: 'country', $id: country1 } });

var country2 = ObjectId();
db.country.insert({ _id: country2, code: 2, name: 'Brasil' });
db.state.insert({ code: 1, name: 'SC', country : { $ref: 'country', $id: country2 } });

var country3 = ObjectId();
db.country.insert({ _id: country3, code: 3, name: 'Mexico' });
db.state.insert({ code: 1, name: 'TA', country : { $ref: 'country', $id: country3 } });
db.state.insert({ code: 2, name: 'NL', country : { $ref: 'country', $id: country3 } });