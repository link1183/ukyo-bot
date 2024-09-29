-- MariaDB dump 10.19  Distrib 10.11.6-MariaDB, for debian-linux-gnu (aarch64)
--
-- Host: localhost    Database: ukyo_bot
-- ------------------------------------------------------
-- Server version	10.11.6-MariaDB-0+deb12u1

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `boot`
--

DROP TABLE IF EXISTS `boot`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `boot` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `user_id` int(11) NOT NULL,
  `score` double NOT NULL,
  `date` datetime NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `id` (`id`),
  KEY `boot_fk1` (`user_id`),
  CONSTRAINT `boot_fk1` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=128 DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `boot`
--

LOCK TABLES `boot` WRITE;
/*!40000 ALTER TABLE `boot` DISABLE KEYS */;
INSERT INTO `boot` VALUES
(1,1,0.8843983398846806,'2024-09-23 11:35:18'),
(2,1,0.6615576871325399,'2024-09-23 11:40:43'),
(4,3,0.661938770031073,'2024-09-23 11:44:38'),
(5,1,0.5869420020166796,'2024-09-23 13:39:08'),
(6,4,0.747568660547284,'2024-09-23 14:33:09'),
(7,4,0.16978108448357765,'2024-09-23 14:35:19'),
(8,4,0.17827722951016245,'2024-09-23 14:35:21'),
(9,3,0.5108263282552884,'2024-09-23 16:15:37'),
(10,5,0.8989739038717329,'2024-09-23 16:19:59'),
(11,3,0.19082596644092165,'2024-09-23 16:34:57'),
(12,3,0.23744653888152545,'2024-09-23 16:59:55'),
(13,3,0.25220747866063775,'2024-09-23 17:16:41'),
(14,3,0.6615345700373579,'2024-09-23 17:17:45'),
(15,3,0.4739659385817704,'2024-09-23 17:21:16'),
(16,3,0.16108525921263062,'2024-09-23 17:21:26'),
(17,3,0.4574336298073367,'2024-09-23 17:25:01'),
(18,3,0.6995363104388229,'2024-09-23 17:25:46'),
(19,3,0.9975686779644591,'2024-09-23 17:26:09'),
(20,3,0.1685885156381084,'2024-09-23 17:30:04'),
(21,1,0.9564753280246954,'2024-09-23 17:32:52'),
(23,6,0.5709438080802159,'2024-09-23 17:40:30'),
(24,3,0.8378104002343295,'2024-09-23 17:57:13'),
(25,6,0.011062572651197922,'2024-09-23 18:06:00'),
(26,6,0.9273494441012664,'2024-09-23 18:06:56'),
(27,6,0.44496994409325397,'2024-09-23 18:07:03'),
(28,6,0.4989442342711934,'2024-09-23 18:12:46'),
(29,3,0.8774723463949454,'2024-09-23 20:28:25'),
(30,6,0.30425507735677404,'2024-09-23 21:35:39'),
(31,7,0.5662258726441729,'2024-09-23 21:53:12'),
(32,8,0.9476831002034062,'2024-09-23 22:03:40'),
(33,6,0.5777150913119279,'2024-09-24 02:29:01'),
(34,6,0.12117894970334464,'2024-09-24 02:29:23'),
(35,6,0.18843693477416312,'2024-09-24 02:29:27'),
(36,6,0.20203428729563522,'2024-09-24 02:29:43'),
(37,6,0.3193032013707996,'2024-09-24 02:29:47'),
(38,6,0.5172297477363936,'2024-09-24 02:29:50'),
(39,6,0.5203244207981902,'2024-09-24 02:29:54'),
(40,6,0.4841434072035288,'2024-09-24 02:29:58'),
(41,1,0.8916987650211167,'2024-09-24 07:39:21'),
(42,8,0.8240195224353474,'2024-09-24 07:39:49'),
(43,1,0.5641184925302282,'2024-09-24 07:50:15'),
(44,1,0.7159679268731429,'2024-09-24 08:14:31'),
(45,1,0.6925412618961401,'2024-09-24 08:16:52'),
(46,3,0.35466164917731047,'2024-09-24 09:12:28'),
(47,8,0.6831966540116534,'2024-09-24 09:17:00'),
(48,1,0.671479610870309,'2024-09-24 09:53:53'),
(49,3,0.19635679469360334,'2024-09-24 09:53:55'),
(50,3,0.8527835824485726,'2024-09-24 09:54:05'),
(51,3,0.6274652011243531,'2024-09-24 09:58:59'),
(52,1,0.9133834580006718,'2024-09-24 10:04:53'),
(53,3,0.33636105323423093,'2024-09-24 10:14:27'),
(54,6,0.8031779873679135,'2024-09-24 10:50:12'),
(55,6,0.580954356315795,'2024-09-24 10:50:18'),
(56,1,0.8492382281626414,'2024-09-24 11:47:44'),
(57,4,0.7466585585104757,'2024-09-24 12:05:45'),
(58,3,0.9185271974698647,'2024-09-24 16:56:18'),
(59,6,0.7924853909657448,'2024-09-24 20:01:29'),
(60,3,0.1268132825966748,'2024-09-24 20:01:47'),
(61,1,0.6521723425405827,'2024-09-24 20:22:48'),
(62,3,0.2878076792603588,'2024-09-24 20:26:14'),
(63,6,0.8388779077122984,'2024-09-24 20:26:21'),
(64,3,0.5516838112639345,'2024-09-24 20:27:05'),
(65,3,0.5990684262432837,'2024-09-24 20:27:38'),
(66,6,0.577098365521162,'2024-09-24 20:27:55'),
(67,3,0.3622944560652589,'2024-09-24 20:28:08'),
(68,1,0.6152991529284663,'2024-09-25 08:14:44'),
(69,1,0.7525456201600726,'2024-09-25 09:12:43'),
(70,3,0.47894691355113206,'2024-09-25 10:44:24'),
(71,1,0.36162701395455854,'2024-09-25 11:05:49'),
(72,3,0.04125576888971971,'2024-09-25 13:57:24'),
(73,5,0.6295168367982886,'2024-09-25 14:47:19'),
(74,8,0.31167218112903916,'2024-09-25 16:22:48'),
(75,8,0.39548109623335415,'2024-09-25 16:22:53'),
(76,8,0.9199211633844433,'2024-09-25 16:22:56'),
(77,3,0.5347053764284388,'2024-09-26 13:03:47'),
(78,1,0.9185235618900346,'2024-09-26 13:05:14'),
(79,3,0.3804483481230445,'2024-09-26 13:05:36'),
(80,3,0.7015569146895443,'2024-09-26 13:06:19'),
(81,6,0.263401911434364,'2024-09-26 13:08:42'),
(82,6,0.27501751942824615,'2024-09-26 13:09:04'),
(83,8,0.5099661474989367,'2024-09-26 14:59:39'),
(84,8,0.5797308992958217,'2024-09-26 14:59:48'),
(85,8,0.3393210781214685,'2024-09-26 14:59:53'),
(86,3,0.5250159232587972,'2024-09-26 15:00:07'),
(87,3,0.3666267526234631,'2024-09-26 15:00:14'),
(88,1,0.43957104442558337,'2024-09-26 15:01:03'),
(89,6,0.43322919607002075,'2024-09-26 18:03:21'),
(90,3,0.5656495954782019,'2024-09-26 18:03:31'),
(91,3,0.29295109151726817,'2024-09-26 20:58:50'),
(92,3,0.6878839659332724,'2024-09-26 20:58:58'),
(93,3,0.8346679205957395,'2024-09-26 21:01:06'),
(94,11,0.040569202328200604,'2024-09-26 21:01:16'),
(95,3,0.3543783987043968,'2024-09-26 21:03:47'),
(96,6,0.9741896480799817,'2024-09-26 23:40:32'),
(97,1,0.4918519511472187,'2024-09-27 11:35:42'),
(98,8,0.8153677126158545,'2024-09-27 12:41:37'),
(99,8,0.7320905911156244,'2024-09-27 12:41:50'),
(100,8,0.6205025270778457,'2024-09-27 12:41:59'),
(101,8,0.09408275736200489,'2024-09-27 12:42:12'),
(102,3,0.32653917511323316,'2024-09-27 13:50:17'),
(103,3,0.0025323780148093222,'2024-09-27 13:50:23'),
(104,3,0.8524348777668659,'2024-09-27 20:59:46'),
(105,3,0.972929631911581,'2024-09-27 21:00:02'),
(106,3,0.3423344634261918,'2024-09-27 21:00:57'),
(107,3,0.82269159927218,'2024-09-27 21:01:03'),
(108,3,0.5776823354904354,'2024-09-27 21:01:09'),
(109,3,0.4046327790914904,'2024-09-27 21:01:14'),
(110,3,0.42583950399148873,'2024-09-27 21:01:19'),
(111,3,0.31968707305418687,'2024-09-27 21:01:24'),
(112,3,0.17447557285970583,'2024-09-27 21:01:29'),
(113,3,0.4525574559246891,'2024-09-27 21:02:40'),
(114,3,0.3208090674880746,'2024-09-27 21:02:44'),
(115,3,0.12219057435550873,'2024-09-27 21:02:47'),
(116,3,0.1921913887716572,'2024-09-27 21:02:57'),
(117,3,0.19659356597049893,'2024-09-27 21:03:08'),
(118,3,0.5259768484321724,'2024-09-27 21:03:35'),
(119,1,0.007800446326854504,'2024-09-28 07:45:38'),
(120,3,0.36715248538727274,'2024-09-28 17:48:05'),
(121,3,0.599166539621189,'2024-09-28 17:48:12'),
(122,8,0.7900326533609916,'2024-09-28 20:01:32'),
(123,3,0.6373859800808116,'2024-09-28 20:02:12'),
(124,6,0.5579791137364467,'2024-09-28 23:46:15'),
(125,8,0.5514223874733034,'2024-09-29 00:28:12'),
(126,1,0.05187407283987222,'2024-09-29 12:43:00'),
(127,3,0.1734410346915226,'2024-09-29 12:43:05');
/*!40000 ALTER TABLE `boot` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `messages`
--

DROP TABLE IF EXISTS `messages`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `messages` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `message` varchar(200) NOT NULL,
  `suggestion_date` datetime NOT NULL,
  `user_id` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `id` (`id`),
  KEY `messages_fk3` (`user_id`),
  CONSTRAINT `messages_fk3` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `messages`
--

LOCK TABLES `messages` WRITE;
/*!40000 ALTER TABLE `messages` DISABLE KEYS */;
INSERT INTO `messages` VALUES
(2,'Hey {user} welcome!','2024-09-25 09:02:36',1),
(3,'A wild {user} has joined the server','2024-09-25 09:10:43',1),
(4,'Wassup {user}','2024-09-25 09:14:30',1),
(5,'Wassup {user}','2024-09-25 09:14:30',1),
(6,'Welcome awesome person!','2024-09-25 10:46:23',9);
/*!40000 ALTER TABLE `messages` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `users`
--

DROP TABLE IF EXISTS `users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `discord_id` bigint(20) unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `id` (`id`),
  UNIQUE KEY `discord_id` (`discord_id`)
) ENGINE=InnoDB AUTO_INCREMENT=12 DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `users`
--

LOCK TABLES `users` WRITE;
/*!40000 ALTER TABLE `users` DISABLE KEYS */;
INSERT INTO `users` VALUES
(4,238382200305876993),
(9,258772567433805824),
(8,283305414362267648),
(1,394953779406962690),
(11,399046814080172046),
(5,411676990471667712),
(6,588751067198521370),
(3,807296732370960404),
(7,870549568616538172),
(2,1272667516178534421);
/*!40000 ALTER TABLE `users` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2024-09-29 14:56:02