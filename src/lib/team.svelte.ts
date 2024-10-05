let teamName = $state('');
let teamId = $state('');
let teamDefault = $state(false);

export function teamState() {
  function setTeamName(name: string) {
    teamName = name;
  }

  function setTeamId(id: string) {
    teamId = id;
  }

  function setTeamDefault(defaut: boolean) {
    teamDefault = defaut;
  }

  return {
    get teamName() {
      return teamName;
    },
    get teamId() {
      return teamId;
    },
    get teamDefault() {
      return teamDefault;
    },
    setTeamName,
    setTeamId,
    setTeamDefault,
  };
}
