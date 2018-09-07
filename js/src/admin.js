class Admin {
  constructor(user) {
    this.user = user;
  }

  // Facade
  ldapLogin() {
    return this.user.ldapLogin();
  }
}

module.exports = Admin;
