class User {
  constructor(data) {
    this.data = data;
  }

  name() {
    return this.data.name;
  }
}

module.exports = User;
