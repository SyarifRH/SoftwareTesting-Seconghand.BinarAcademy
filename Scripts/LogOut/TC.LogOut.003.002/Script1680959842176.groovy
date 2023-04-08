import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://secondhand.binaracademy.org/')

WebUI.click(findTestObject('Object Repository/LogOut/Page_SecondHand/a_Masuk'))

WebUI.setText(findTestObject('Object Repository/LogOut/Page_SecondHand/input_Email_useremail'), 'testdata@testdata.com')

WebUI.setEncryptedText(findTestObject('Object Repository/LogOut/Page_SecondHand/input_Password_userpassword'), '0ZVc8hhn6Ko=')

WebUI.click(findTestObject('Object Repository/LogOut/Page_SecondHand/input_Password_commit'))

WebUI.navigateToUrl('https://secondhand.binaracademy.org/logout')

WebUI.verifyElementVisible(findTestObject('LogOut/Page_SecondHand/a_Masuk'))

WebUI.closeBrowser()

