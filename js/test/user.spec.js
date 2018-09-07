const chai = require('chai');
const { expect } = chai;

const User = require('../src/user');

describe('User', () => {
  beforeEach(() => {
    this.data = {
      name: 'john',
    }
  });
  context('Member', () => {
    it('should have a name', () => {
      const user = new User(this.data);
      expect(user.name()).to.eq('john');
    });
  });
});
