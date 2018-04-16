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
  width: window.innerWidth,
  height: window.innerHeight,
  physics: {
    default: 'arcade',
  },
  scene: GameScene,
}

// Kickstart the game
new Phaser.Game(config)
