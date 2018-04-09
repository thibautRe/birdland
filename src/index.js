import Phaser from 'phaser'

import GameScene from './scene'

// Fix hot reloading by reloading the page completely
if (module.hot) {
  module.hot.accept(() => {
    window.location.reload()
  })
}

const config = {
    type: Phaser.AUTO,
    width: 800,
    height: 600,
    physics: {
      default: 'arcade',
    },
    scene: GameScene,
}

const game = new Phaser.Game(config);
