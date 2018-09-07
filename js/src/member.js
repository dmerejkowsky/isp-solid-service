class Member {
  constructor(user) {
    this.user = user;
  }

  // Extraction
  address() {
    const { address1, address2 } = this.user.data;
    return `${address1}\n${address2}`;
  }

  name() {
      return this.data.name;
  }
}

module.exports = Member;
