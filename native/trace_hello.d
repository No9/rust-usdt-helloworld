
provider myserv {
	probe query__receive(string, string);
	probe query__respond();
};
