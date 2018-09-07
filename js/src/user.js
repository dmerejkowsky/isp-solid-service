class User {
  constructor(data) {
    this.data = data;
  }

  name() {
      return this.data.name;
  }

  ldapLogin() {
    const { enterpriseName, name } = this.data;
    return `${enterpriseName}/admin/${name}`;
  }

  daysLeft() {
    const now = new Date().getTime();
    const { createdAt } = this.data;
    return this._fromMilis(now - createdAt);
  }

  _fromMilis(milis) {
    return Math.floor(milis / (1000 * 60 * 60 * 24));
  }
}


module.exports = User;
