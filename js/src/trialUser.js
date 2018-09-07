const User = require('./user');

// Classic Inheritance
class TrialUser extends User {
  name() {
      return this.data.tempLogin;
  }
}

module.exports = TrialUser;
