import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('')

WebUI.navigateToUrl('https://bwae.test.bwae.org/#/signin?to=%2F')

WebUI.click(findTestObject('AE/Page_/body'))

WebUI.clickOffset(findTestObject('AE/Page_/div__IronCurtain'), 2, 3)

WebUI.setText(findTestObject('Object Repository/AE2/Page_/input__loginName'), 'admin-gogs')

WebUI.setEncryptedText(findTestObject('Object Repository/AE2/Page_/input__password'), '8sHRIeIYQYM=')

WebUI.click(findTestObject('Object Repository/AE2/Page_/a_'))

WebUI.closeBrowser()

