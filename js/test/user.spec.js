const chai = require('chai');
const { expect } = chai;

const User = require('../src/user');

describe('User', () => {
  beforeEach(() => {
    this.data = {
      name: 'john',
      enterpriseName: 'fooCorp',
    }
  });

  it('should have a name', () => {
    const user = new User(this.data);
    expect(user.name()).to.eq('john');
  });

  context('Member', () => {
    it('should have an address', () => {
      this.data.member = true;
      this.data.address1 = '452 Wilson Summit';
      this.data.address2 = 'East Dawnshier, AK 96919';
      const user = new User(this.data);
      expect(user.address()).to.eq('452 Wilson Summit\nEast Dawnshier, AK 96919');
    });
  });

  context('Admin', () => {
    it('should have a LDAP login', () => {
      const user = new User(this.data);
      this.data.admin = true;
      this.data.enterpriseName = 'fooCorp';
      expect(user.ldapLogin()).to.eq('fooCorp/admin/john');
    });
  });

  context('Trial User', () => {
    const now = new Date().getTime();
    const twoDaysAgo = new Date(now - 1000 * 60 * 60 * 24 * 2);

    beforeEach(() => {
      this.data.trial = true;
      this.data.tempLogin = 'temp login';
      this.data.createdAt = twoDaysAgo;
    });

    it('should have a temporary name', () => {
      const user = new User(this.data);
      expect(user.name()).to.eq('temp login');
    });

    it('should compute the number of days left', () => {
      const user = new User(this.data);
      expect(user.name()).to.eq('temp login');
      expect(user.daysLeft()).to.eq(2);
    });
  });


});
