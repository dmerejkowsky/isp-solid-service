const chai = require('chai');
const { expect } = chai;

const User = require('../src/user');
const Member = require('../src/member');
const Admin = require('../src/admin');
const TrialUser = require('../src/trialUser');

describe('users', () => {
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
    beforeEach(() => {
      const user = new User(this.data);
      this.member = new Member(user);
    });
    it('should have an address', () => {
      this.data.address1 = '452 Wilson Summit';
      this.data.address2 = 'East Dawnshier, AK 96919';
      expect(this.member.address()).to.eq('452 Wilson Summit\nEast Dawnshier, AK 96919');
    });
  });

  context('Admin', () => {
    beforeEach(() => {
      const user = new User(this.data);
      this.admin = new Admin(user);
    });
    it('should have a LDAP login', () => {
      this.data.enterpriseName = 'fooCorp';
      expect(this.admin.ldapLogin()).to.eq('fooCorp/admin/john');
    });
  });

  context('Trial User', () => {

    beforeEach(() => {
      const now = new Date().getTime();
      const twoDaysAgo = new Date(now - 1000 * 60 * 60 * 24 * 2);
      this.data.tempLogin = 'temp login';
      this.data.createdAt = twoDaysAgo;
      this.trialUser = new TrialUser(this.data);
    });

    it('should have a temporary name', () => {
      const user = new TrialUser(this.data);
      expect(user.name()).to.eq('temp login');
    });

    it('should compute the number of days left', () => {
      const user = new TrialUser(this.data);
      expect(user.daysLeft()).to.eq(2);
    });
  });


});
