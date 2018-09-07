const chai = require('chai');
const { expect } = chai;

const User = require('../src/user');

describe('User', () => {
  beforeEach(() => {
    this.data = {
      name: 'john',
      address1: '452 Wilson Summit',
      address2: 'East Dawnshier, AK 96919',
    }
  });

  it('should have a name', () => {
    const user = new User(this.data);
    expect(user.name()).to.eq('john');
  });

  context('Member', () => {

    it('should have an address', () => {
      const user = new User(this.data);
      expect(user.address()).to.eq('452 Wilson Summit\nEast Dawnshier, AK 96919');
    });
  });

});
