class User {
  constructor(data) {
    this.data = data;
  }

  name() {
    return this.data.name;
  }

  address() {
    const { address1, address2 } = this.data;
    return `${address1}\n${address2}`;
  }
}

module.exports = User;
